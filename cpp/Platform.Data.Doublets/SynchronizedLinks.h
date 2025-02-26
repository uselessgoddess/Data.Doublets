﻿namespace Platform::Data::Doublets
{
    template <typename ...> class SynchronizedLinks;
    template <typename TLinkAddress> class SynchronizedLinks<TLinkAddress> : public ISynchronizedLinks<TLinkAddress>
    {
        public: const LinksConstants<TLinkAddress> Constants;

        public: const ISynchronization *SyncRoot;

        public: const ILinks<TLinkAddress> *Sync;

        public: const ILinks<TLinkAddress> *Unsync;

        public: SynchronizedLinks(ILinks<TLinkAddress> &links) : this(ReaderWriterLockSynchronization(), links) { }

        public: SynchronizedLinks(ISynchronization &synchronization, ILinks<TLinkAddress> &links)
        {
            SyncRoot = synchronization;
            Sync = this;
            Unsync = links;
            Constants = links.Constants;
        }

        public: TLinkAddress Count(IList<TLinkAddress> &restriction) { return SyncRoot.ExecuteReadOperation(restriction, Unsync.Count()); }

        public: TLinkAddress Each(Func<IList<TLinkAddress>, TLinkAddress> handler, IList<TLinkAddress> &restrictions) { return SyncRoot.ExecuteReadOperation(handler, restrictions, (handler1, restrictions1) { return Unsync.Each(handler1, restrictions1)); } }

        public: TLinkAddress Create(IList<TLinkAddress> &restrictions) { return SyncRoot.ExecuteWriteOperation(restrictions, Unsync.Create); }

        public: TLinkAddress Update(IList<TLinkAddress> &restrictions, IList<TLinkAddress> &substitution) { return SyncRoot.ExecuteWriteOperation(restrictions, substitution, Unsync.Update); }

        public: void Delete(IList<TLinkAddress> &restrictions) { SyncRoot.ExecuteWriteOperation(restrictions, Unsync.Delete); }
    };
}
