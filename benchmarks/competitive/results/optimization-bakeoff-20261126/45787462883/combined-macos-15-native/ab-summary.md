| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.942x | 0.710x | 1.223x |
| materialize | 0.960x | 0.775x | 0.984x |
| perft_bulk_d4 | 0.813x | 0.743x | 1.122x |
| perft_full_d3 | 0.829x | 0.718x | 1.213x |
| fen_parse | 0.988x | 0.849x | 1.163x |
| fen_write_native_ep | 0.874x | 0.681x | 1.925x |
| snapshot | 0.879x | 0.735x | 0.882x |
| san_parse_all | 0.907x | 0.803x | 1.966x |
| san_render_native | 0.902x | 0.770x | 2.883x |
| apply_children_full | 0.676x | 0.566x | 0.673x |
| make_unmake_full | 0.522x | 0.496x | 0.641x |
| startpos | 0.754x | 0.754x | 0.933x |
| chess960_materialize | 1.053x | 1.016x | 1.585x |
| chess960_perft_d3 | 0.820x | 0.732x | 1.275x |
| polyglot_key | 0.816x | 0.749x | 34.214x |
| polyglot_recompute | 0.890x | 0.867x | 0.925x |
| repetition_key | 0.837x | 0.828x | 21.999x |
| corpus_count | 0.893x | 0.769x | 1.491x |
| corpus_materialize | 0.857x | 0.697x | 1.144x |
| corpus_uci_sorted | 0.852x | 0.819x | 1.381x |
| replay_trusted_native | 0.704x | 0.575x | 0.434x |
| replay_checked_native | 0.817x | 0.797x | 0.713x |

Cold first legal move list: 0.013 ms -> 0.013 ms (1.03x).
