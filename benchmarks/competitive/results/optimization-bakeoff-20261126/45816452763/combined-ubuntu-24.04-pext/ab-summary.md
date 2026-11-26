| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.000x | 0.978x | 1.386x |
| materialize | 1.010x | 0.999x | 1.515x |
| perft_bulk_d4 | 1.000x | 0.988x | 1.067x |
| perft_full_d3 | 0.916x | 0.854x | 1.349x |
| fen_parse | 1.058x | 0.973x | 1.111x |
| fen_write_native_ep | 1.018x | 0.981x | 1.935x |
| snapshot | 0.999x | 0.996x | 1.331x |
| san_parse_all | 1.026x | 1.013x | 1.940x |
| san_render_native | 1.017x | 0.996x | 2.127x |
| apply_children_full | 0.973x | 0.918x | 1.085x |
| make_unmake_full | 0.912x | 0.879x | 1.015x |
| startpos | 1.254x | 1.254x | 1.256x |
| chess960_materialize | 0.867x | 0.792x | 1.231x |
| chess960_perft_d3 | 0.955x | 0.919x | 1.320x |
| polyglot_key | 1.068x | 1.000x | 41.117x |
| polyglot_recompute | 0.991x | 0.988x | 1.049x |
| repetition_key | 1.058x | 1.008x | 17.326x |
| corpus_count | 1.006x | 0.992x | 0.926x |
| corpus_materialize | 1.004x | 1.000x | 1.065x |
| corpus_uci_sorted | 1.004x | 1.002x | 1.564x |
| replay_trusted_native | 0.928x | 0.839x | 0.766x |
| replay_checked_native | 0.963x | 0.896x | 1.086x |

Cold first legal move list: 0.005 ms -> 0.006 ms (0.88x).
