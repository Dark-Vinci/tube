import React, { useState } from 'react';
import {
  View,
  Text,
  ScrollView,
  FlatList,
  TextInput,
  Image,
  TouchableOpacity,
  StyleSheet,
} from 'react-native';

import {
  Comment,
  commentProp,
  IfElse,
  LetterProfile,
  Send,
  Close,
} from '@components';

export interface commentProps {
  comments: commentProp[];
  profileUrl: string | null;
}

export function Comments({ comments, profileUrl }: commentProps): JSX.Element {
  const [comment, setComment] = useState<string>('');

  return (
    <View style={style.container}>
      {/* header */}
      <View>
        {/* overline */}
        <View>
          <Text>Overline</Text>
        </View>

        {/*  */}
        <View>
          <View>
            <Text>Comments</Text>
            {/* close icons */}
            <Close />
          </View>
        </View>

        {/* ordering */}
        <View>
          <Text>Top</Text>
          <Text>Timed</Text>
          <Text>Newest</Text>
        </View>
      </View>

      {/* body */}
      <ScrollView>
        {/* should update list when we scroll to the last 3 items */}
        <FlatList
          data={comments}
          renderItem={({ item }) => {
            return <Comment {...item} />;
          }}
          keyExtractor={(item) => item.id}
        />
      </ScrollView>

      {/* footer */}
      <View>
        {/* NAME */}
        <IfElse
          condition={!!profileUrl}
          ifElement={<Image src={profileUrl!} />}
          elseElement={<LetterProfile letter="o" />}
        />

        {/* INPUT */}
        <TextInput
          maxLength={40}
          onChangeText={(text) => setComment(text)}
          value={comment}
        />

        {/* SEND */}
        <TouchableOpacity
          onPress={() => console.log({ comment, action: 'make comment' })}
        >
          <Send isActive={true} />
        </TouchableOpacity>
      </View>
    </View>
  );
}

const style = StyleSheet.create({
  container: {},
});
