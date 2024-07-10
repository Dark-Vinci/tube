import React, { SetStateAction, Dispatch } from 'react';

import { View, TextInput, TouchableOpacity, Image } from 'react-native';

import { IfElse } from '../../conditionals';
import { LetterProfile } from '../../Ui';
import { Send } from '../../icons';

export interface sendCommentProps {
  profileUrl: string;
  comment: string;
  setComment: Dispatch<SetStateAction<string>>;
  username: string;
}

export function SendComment({
  profileUrl,
  username,
  comment,
  setComment,
}: sendCommentProps): JSX.Element {
  return (
    <View>
      {/* NAME */}
      <IfElse
        condition={!!profileUrl}
        ifElement={<Image src={profileUrl!} />}
        elseElement={<LetterProfile letter={username[0]} />}
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
  );
}
