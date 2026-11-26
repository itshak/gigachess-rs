| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.056x | 1.010x | 1.320x |
| materialize | 1.031x | 0.986x | 1.087x |
| perft_bulk_d4 | 1.023x | 1.017x | 0.986x |
| perft_full_d3 | 0.917x | 0.876x | 0.565x |
| fen_parse | 1.106x | 1.086x | 1.238x |
| fen_write_native_ep | 1.001x | 0.985x | 1.198x |
| snapshot | 1.378x | 1.269x | 1.091x |
| san_parse_all | 1.044x | 1.035x | 1.028x |
| san_render_native | 1.029x | 1.006x | 1.159x |
| apply_children_full | 1.120x | 1.090x | 0.810x |
| make_unmake_full | 1.033x | 1.027x | 0.863x |
| startpos | 1.420x | 1.420x | 1.080x |
| chess960_materialize | 0.959x | 0.946x | 1.206x |
| chess960_perft_d3 | 1.051x | 1.042x | 1.056x |
| polyglot_key | 0.927x | 0.857x | 20.836x |
| polyglot_recompute | 0.942x | 0.932x | 0.995x |
| repetition_key | 1.165x | 0.922x | 15.150x |
| corpus_count | 1.045x | 1.030x | 1.292x |
| corpus_materialize | 1.016x | 1.006x | 1.168x |
| corpus_uci_sorted | 0.958x | 0.954x | 0.617x |
| replay_trusted_native | 1.014x | 0.965x | 0.535x |
| replay_checked_native | 1.044x | 1.030x | 0.748x |

Cold first legal move list: 153.393 ms → 161.743 ms (0.95x).
