#!/usr/bin/python3

URL = 'https://dota2.gamepedia.com/Chat_Wheel'
FNAME = './data/chatwheel.json'

def packageid(text):
    if 'international' in text.lower():
        year = text.split(' ')[-1]
        return 'ti' + year
    return text.lower().replace(' ', '_')

def contains_chatwheel(table):
    #cond = any(map(lambda th: th.text.strip() == 'Lines', table.find_all('th')))
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

    def is_tooltip(item):
        return 'class' in item.attrs and 'tooltip' in item.attrs['class']

    cmd, content = table.find_all('code'), table.find_all('span')
    content = group(content)

    for cmd, content in zip(cmd, content.values()):
        try:
            _, cid = cmd.text.strip().split(' ')
            parent = content[0].parent

            audios = (item.extract() for item in parent.find_all('span') if not is_tooltip(item))
            audios = list(map(lambda node: node.find('source').attrs['src'], audios))
            text = parent.text.strip()

            results[f'{pid}_{cid}'] = {
                'text': text,
                'audios': audios,
            }

            print(cid, text, 'audios:', len(audios))

        except ValueError:
            print('skipping', cmd)

    return results

def main():
    import json
    import sys
    from bs4 import BeautifulSoup
    from urllib.request import urlopen

    save_enabled = '--save' in sys.argv

    response = urlopen(URL)
    if response.status != 200:
        raise Exception(response.status_code)

    pagesource = response.read()
    soup = BeautifulSoup(pagesource, 'html.parser')
    tables = list(filter(contains_chatwheel, soup.find_all('table')))
    chatwheel = {}

    for items in map(extract, tables):
        chatwheel.update(items)

    print('=== total chatwheel items:', len(chatwheel.keys()))

    if save_enabled:
        print(f'saving to {FNAME}...')
        with open(FNAME, 'w') as fout:
            fout.write(json.dumps(chatwheel))
        print('done.')

if __name__ == '__main__':
    main()
