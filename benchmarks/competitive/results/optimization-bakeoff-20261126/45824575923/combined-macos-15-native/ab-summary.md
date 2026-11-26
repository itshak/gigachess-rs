| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.052x | 0.610x | 1.361x |
| materialize | 1.075x | 0.997x | 1.083x |
| perft_bulk_d4 | 1.165x | 0.788x | 1.241x |
| perft_full_d3 | 1.064x | 1.020x | 1.416x |
| fen_parse | 1.151x | 0.958x | 1.252x |
| fen_write_native_ep | 1.061x | 0.959x | 1.788x |
| snapshot | 1.095x | 0.825x | 0.906x |
| san_parse_all | 0.983x | 0.907x | 1.741x |
| san_render_native | 0.896x | 0.582x | 2.585x |
| apply_children_full | 1.021x | 0.840x | 1.062x |
| make_unmake_full | 1.037x | 0.906x | 1.181x |
| startpos | 0.617x | 0.617x | 0.898x |
| chess960_materialize | 1.213x | 1.049x | 1.262x |
| chess960_perft_d3 | 1.182x | 1.101x | 1.653x |
| polyglot_key | 0.875x | 0.849x | 34.149x |
| polyglot_recompute | 0.970x | 0.880x | 1.009x |
| repetition_key | 0.986x | 0.896x | 19.206x |
| corpus_count | 1.134x | 1.047x | 1.087x |
| corpus_materialize | 0.968x | 0.946x | 1.173x |
| corpus_uci_sorted | 1.047x | 1.037x | 1.268x |
| replay_trusted_native | 0.986x | 0.940x | 0.588x |
| replay_checked_native | 0.962x | 0.903x | 1.163x |

Cold first legal move list: 0.008 ms -> 0.005 ms (1.65x).
