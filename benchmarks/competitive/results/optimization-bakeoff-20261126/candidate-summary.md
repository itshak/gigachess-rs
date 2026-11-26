| Candidate | Primary workload | x64 ratio | ARM64 ratio | Evidence |
|---|---|---:|---:|---|
| mutation | make_unmake_full | 1.04x | 1.03x | both |
| undo-role | perft_full_d3 | 0.97x | 1.03x | both |
| ray-prefilter | replay_checked_native | 1.09x | 1.12x | both |
| king-targets | corpus_count | 0.95x | 0.87x | both |
| copy-perft | perft_full_d3 | 1.20x | 1.05x | both |
| fused-perft | perft_bulk_d4 | 1.09x | 1.10x | both |
| san | san_parse_all | 1.53x | 1.43x | both |
| uci | corpus_uci_sorted | 1.21x | 1.31x | both |
| standard-castle | apply_children_full | 1.12x | 0.98x | both |
| role-lookup | perft_full_d3 | 0.78x | 0.84x | both |
| static-sliders | corpus_count | 0.97x | 1.14x | both |
| compact-board | snapshot | 1.38x | 1.23x | both |
| mailbox | apply_children_full | 0.90x | 0.92x | both |
| direct-legal | san_parse_all | 1.69x | 1.85x | both |
| early-san | san_render_native | 1.61x | 1.58x | both |
| standard-movegen | corpus_count | 1.00x | 1.00x | both |
| ep-candidates | corpus_count | 1.01x | 1.02x | both |
| tagged-perft | perft_bulk_d4 | 1.06x | 1.05x | both |
| perft-kernel | perft_full_d3 | 1.04x | 1.06x | both |
| direct-hash | replay_trusted_native | 1.00x | 1.00x | both |
| quiet-fast | replay_trusted_native | 1.17x | 1.09x | both |
| standard-no-special | corpus_count | 1.02x | 1.01x | both |
| king-local | corpus_count | 0.88x | 0.95x | both |
| pin-rays | corpus_count | 1.08x | 1.04x | both |
| fused-geometry | perft_bulk_d4 | 1.11x | 1.22x | both |
| uci-direct | corpus_uci_sorted | 1.55x | 1.70x | both |

Ratios compare candidate to same-runner baseline. Two ABBA repeats; values within ~3% should not be treated as conclusive. See JSON for all operations and worst-case ratios.
