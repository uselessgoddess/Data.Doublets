﻿using System.Collections.Generic;
using System.Runtime.CompilerServices;
using Platform.Converters;
using Platform.Data.Doublets.Sequences.Indexes;

#pragma warning disable CS1591 // Missing XML comment for publicly visible type or member

namespace Platform.Data.Doublets.Unicode
{
    public class StringToUnicodeSequenceConverter<TLink> : LinksOperatorBase<TLink>, IConverter<string, TLink>
    {
        private readonly IConverter<string, IList<TLink>> _stringToUnicodeSymbolListConverter;
        private readonly ISequenceIndex<TLink> _index;
        private readonly IConverter<IList<TLink>, TLink> _listToSequenceLinkConverter;
        private readonly TLink _unicodeSequenceMarker;

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public StringToUnicodeSequenceConverter(ILinks<TLink> links, IConverter<string, IList<TLink>> stringToUnicodeSymbolListConverter, ISequenceIndex<TLink> index, IConverter<IList<TLink>, TLink> listToSequenceLinkConverter, TLink unicodeSequenceMarker) : base(links)
        {
            _stringToUnicodeSymbolListConverter = stringToUnicodeSymbolListConverter;
            _index = index;
            _listToSequenceLinkConverter = listToSequenceLinkConverter;
            _unicodeSequenceMarker = unicodeSequenceMarker;
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public StringToUnicodeSequenceConverter(ILinks<TLink> links, IConverter<char, TLink> charToUnicodeSymbolConverter, ISequenceIndex<TLink> index, IConverter<IList<TLink>, TLink> listToSequenceLinkConverter, TLink unicodeSequenceMarker)
            : this(links, new StringToUnicodeSymbolsListConverter<TLink>(charToUnicodeSymbolConverter), index, listToSequenceLinkConverter, unicodeSequenceMarker) { }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public StringToUnicodeSequenceConverter(ILinks<TLink> links, IConverter<char, TLink> charToUnicodeSymbolConverter, IConverter<IList<TLink>, TLink> listToSequenceLinkConverter, TLink unicodeSequenceMarker)
            : this(links, charToUnicodeSymbolConverter, new Unindex<TLink>(), listToSequenceLinkConverter, unicodeSequenceMarker) { }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public StringToUnicodeSequenceConverter(ILinks<TLink> links, IConverter<string, IList<TLink>> stringToUnicodeSymbolListConverter, IConverter<IList<TLink>, TLink> listToSequenceLinkConverter, TLink unicodeSequenceMarker)
            : this(links, stringToUnicodeSymbolListConverter, new Unindex<TLink>(), listToSequenceLinkConverter, unicodeSequenceMarker) { }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public TLink Convert(string source)
        {
            var elements = _stringToUnicodeSymbolListConverter.Convert(source);
            _index.Add(elements);
            var sequence = _listToSequenceLinkConverter.Convert(elements);
            return _links.GetOrCreate(sequence, _unicodeSequenceMarker);
        }
    }
}
