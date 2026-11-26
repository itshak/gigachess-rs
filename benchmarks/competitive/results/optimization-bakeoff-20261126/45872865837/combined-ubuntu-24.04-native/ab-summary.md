| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.009x | 0.996x | 1.506x |
| materialize | 1.026x | 1.004x | 1.566x |
| perft_bulk_d4 | 1.006x | 0.998x | 1.174x |
| perft_full_d3 | 1.237x | 1.065x | 1.725x |
| fen_parse | 0.926x | 0.836x | 0.980x |
| fen_write_native_ep | 0.986x | 0.955x | 2.101x |
| snapshot | 1.004x | 0.984x | 0.996x |
| san_parse_all | 1.006x | 0.988x | 1.882x |
| san_render_native | 0.987x | 0.978x | 2.148x |
| apply_children_full | 0.970x | 0.939x | 1.313x |
| make_unmake_full | 0.941x | 0.902x | 1.125x |
| startpos | 0.893x | 0.893x | 1.008x |
| chess960_materialize | 0.995x | 0.993x | 1.554x |
| chess960_perft_d3 | 0.994x | 0.987x | 1.498x |
| polyglot_key | 0.937x | 0.887x | 38.342x |
| polyglot_recompute | 0.970x | 0.967x | 0.994x |
| repetition_key | 1.132x | 0.927x | 22.501x |
| corpus_count | 1.022x | 1.010x | 1.103x |
| corpus_materialize | 1.014x | 1.013x | 1.065x |
| corpus_uci_sorted | 1.004x | 0.999x | 1.564x |
| replay_trusted_native | 1.016x | 0.963x | 0.893x |
| replay_checked_native | 1.004x | 0.978x | 1.208x |

Cold first legal move list: 0.006 ms -> 0.005 ms (1.02x).
