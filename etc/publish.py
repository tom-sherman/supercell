#!/usr/bin/env python3
from typing import Optional
from atproto import Client, models
import argparse

def main(user: str, password: str, name: str, description: str, server: str, rkey: Optional[str] = None, image: Optional[str] = None):
    client = Client()
    client.login(user, password)
    avatar_blob = None
    if image:
        with open(image, 'rb') as f:
            avatar_data = f.read()
            avatar_blob = client.upload_blob(avatar_data).blob
    response = client.com.atproto.repo.put_record(models.ComAtprotoRepoPutRecord.Data(
        repo=client.me.did,
        collection=models.ids.AppBskyFeedGenerator,
        rkey=rkey,
        record=models.AppBskyFeedGenerator.Record(
            did=f'did:web:{server}',
            display_name=name,
            description=description,
            avatar=avatar_blob,
            created_at=client.get_current_time_iso(),
        )
    ))
    print('Feed URI :', response.uri)


if __name__ == '__main__':
    parser = argparse.ArgumentParser()
    parser.add_argument("-u", "--user", help="The handle to publish the feed under. Ex: smokesignal.events")
    parser.add_argument("-p", "--password", help="The password for the handle publishing the feed")
    parser.add_argument("-n", "--name", help="The name of the feed. Ex: What's Hot")
    parser.add_argument("-d", "--description", help="The description of the feed. Ex: Top trending content from the whole network")
    parser.add_argument("-i", "--image", default=None, help="The path to the avatar image for the feed. Ex: ./path/to/avatar.jpeg")
    parser.add_argument("-s", "--server", help="The server hostname servicing the feed. Ex: feeds.smokesignal.events")
    parser.add_argument("-r", "--rkey", default=None, help="The rkey of a feed being updated.")
    args = parser.parse_args()
    main(args.user, args.password, args.name, args.description, args.server, args.rkey, args.image)

