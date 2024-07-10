import React, { useState } from 'react';
import { View, TouchableOpacity } from 'react-native';

import {
  // Comment,
  // commentProp,
  // IfElse,
  // LetterProfile,
  Send,
  // Close,
} from '@components';

export function SingleComment(): JSX.Element {
  const [comment, setComment] = useState<string>('');

  return (
    <View>
      {/* header */}
      <View></View>

      {/* body */}

      {/* reply */}
      <TouchableOpacity
        onPress={() => console.log({ comment, action: 'make comment' })}
      >
        <Send isActive={true} />
      </TouchableOpacity>
    </View>
  );
}
