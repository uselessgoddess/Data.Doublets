﻿

using TLink = std::uint32_t;

namespace Platform::Data::Doublets::Memory::Split::Specific
{
    public unsafe class UInt32UnusedLinksListMethods : public UnusedLinksListMethods<TLink>
    {
        private: readonly RawLinkDataPart<TLink>* _links;
        private: readonly LinksHeader<TLink>* _header;

        public: UInt32UnusedLinksListMethods(RawLinkDataPart<TLink>* links, LinksHeader<TLink>* header)
            : base((std::uint8_t*)links, (std::uint8_t*)header)
        {
            _links = links;
            _header = header;
        }

        protected: override ref RawLinkDataPart<TLink> GetLinkDataPartReference(TLink link) { return &_links[link]; }

        protected: override ref LinksHeader<TLink> GetHeaderReference() { return ref *_header; }
    };
}
