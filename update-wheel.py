#!/usr/bin/python3

URL = 'https://dota2.gamepedia.com/Chat_Wheel'
URL_PLUS = 'https://dota2.gamepedia.com/Chat_Wheel/Dota_Plus'
FNAME = './data/chatwheel.json'

def is_tooltip(item):
    return 'class' in item.attrs and 'tooltip' in item.attrs['class']

def extract_text(item):
    import re
    for sup in item.find_all('sup'):
        sup.extract()
    text = item.text.replace('[All]', '')
    return re.sub('\(.*\)', '', text).strip()

def packageid(text):
    if 'international' in text.lower():
        year = text.split(' ')[-1]
        return 'ti' + year
    return text.lower().replace(' ', '_')

def contains_chatwheel(table):
    return any(map(lambda th: th.text.strip() == 'Lines', table.find_all('th')))

def group(content):
    grouped_content = {}
    for item in content:
        pid = hash(item.parent)
        if pid in grouped_content:
            grouped_content[pid].append(item)
        else:
            grouped_content[pid] = [item]
    return grouped_content

def extract(table):
    results = {}

    title = table.find_previous_sibling('h3').find(class_='mw-headline').text
    pid = packageid(title)

    cmd, content = table.find_all('code'), table.find_all('span')
    content = group(content)

    for cmd, content in zip(cmd, content.values()):
        try:
            _, cid = cmd.text.strip().split(' ')
            parent = content[0].parent

            audios = (item.extract() for item in parent.find_all('span') if not is_tooltip(item))
            audios = list(map(lambda node: node.find('source').attrs['src'], audios))
            text = extract_text(parent)

            results[f'{pid}_{cid}'] = {
                'text': text,
                'audios': audios,
            }

            print(cid, text, 'audios:', len(audios))

        except ValueError:
            print('skipping', cmd)

    return results

def load_site(url):
    from bs4 import BeautifulSoup
    from urllib.request import urlopen

    response = urlopen(url)
    if response.status != 200:
        raise Exception(response.status_code)

    pagesource = response.read()
    return BeautifulSoup(pagesource, 'html.parser')

def process_normal():
    soup = load_site(URL)
    tables = list(filter(contains_chatwheel, soup.find_all('table')))
    return map(extract, tables)

def process_plus():
    soup = load_site(URL_PLUS)
    tables = list(filter(contains_chatwheel, soup.find_all('table')))
    hero_counts = {}
    results = {}

    def extract_hero_name(url):
        for part in url.split('/'):
            if '.mp3' in part:
                hero_name = part.split('_')[1]
                return hero_name
        return None

    for table in tables:
        for li in table.find_all('li'):
            spans = [span.extract() for span in li.find_all('span') if not is_tooltip(span)]

            audios = []
            for span in spans:
                source = span.find('source')
                if source:
                    audios.append(source.attrs['src'])

            text = extract_text(li)

            hero_name = extract_hero_name(audios[0])
            assert not hero_name is None

            if not hero_name in hero_counts:
                hero_counts[hero_name] = 1
            else:
                hero_counts[hero_name] += 1

            lid = f'{hero_name}{hero_counts[hero_name]}'


            results[lid] = {
                'text': text,
                'audios': audios,
            }

            print(lid, text, 'audios:', len(audios))

    return [results]

def main():
    import json
    import sys

    save_enabled = '--save' in sys.argv

    chatwheel = {}

    for items in process_normal():
        chatwheel.update(items)

    for items in process_plus():
        chatwheel.update(items)

    print('=== total chatwheel items:', len(chatwheel.keys()))

    if save_enabled:
        print(f'saving to {FNAME}...')
        with open(FNAME, 'w') as fout:
            fout.write(json.dumps(chatwheel))
        print('done.')

if __name__ == '__main__':
    main()
