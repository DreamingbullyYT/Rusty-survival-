

### Atlas declarations groundwork
- [x] Add exported validation/snapshot helper for mod-owned atlas declarations, enabled-owner filtering, deterministic ordering, duplicate rejection and 32-entry cap.
- [x] Package CI run `44467917911` passed.
- [ ] Repair PMCS failure `44467917951` by documenting this implementation in all required project-memory files.
- [ ] Integrate declarations into the direct, batched Core-facing API and re-submit during each verified Horizon resource reload hook; do not claim live atlas registration yet.