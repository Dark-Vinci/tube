import { View, Image, Text, Linking } from 'react-native';
import React from 'react';

import { If, Like, DisLike, CommentIcon, Menu } from '@components';

export interface commentProp {
  profileUrl: string;
  createdAt: Date;
  content: string;
  username: string;
  replyCount: number;
  id: string;
}

export function Comment({
  username,
  createdAt,
  content,
  replyCount,
  profileUrl,
}: commentProp): JSX.Element {
  return (
    <View>
      <View>
        {/* profile */}
        <View>
          <Image src={profileUrl} />
        </View>

        {/* body */}
        <View>
          <View>
            <Text>{username}</Text>
            <Text>{createdAt.toString()}</Text>
          </View>

          <View>
            {/* format with elipsis */}
            <Text>{content}</Text>
          </View>

          {/* icon */}
          <View>
            <View>
              {/* like */}
              <View>
                <Like /> <Text>{replyCount}</Text>
              </View>

              {/* dislike */}
              <DisLike />

              {/* comment */}
              <CommentIcon />
            </View>
          </View>

          <If
            condition={replyCount > 0}
            element={
              <View>
                <Text
                  onPress={() => Linking.openURL('')}
                >{`${replyCount} ${replyCount > 1 ? 'replies' : 'reply'}`}</Text>
              </View>
            }
          />
        </View>
      </View>

      {/* menu to report */}
      <View>
        <Menu />
      </View>
    </View>
  );
}
