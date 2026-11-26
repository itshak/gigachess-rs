| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.997x | 0.979x | 1.113x |
| materialize | 1.094x | 1.045x | 1.556x |
| perft_bulk_d4 | 1.118x | 1.093x | 1.076x |
| perft_full_d3 | 1.430x | 1.361x | 0.898x |
| fen_parse | 1.080x | 1.029x | 1.052x |
| fen_write_native_ep | 1.014x | 1.001x | 1.267x |
| snapshot | 1.900x | 1.895x | 1.001x |
| san_parse_all | 2.230x | 2.100x | 1.739x |
| san_render_native | 2.528x | 2.203x | 2.143x |
| apply_children_full | 2.255x | 2.088x | 0.947x |
| make_unmake_full | 1.288x | 1.230x | 0.975x |
| startpos | 1.717x | 1.717x | 1.287x |
| chess960_materialize | 1.136x | 1.042x | 1.331x |
| chess960_perft_d3 | 1.193x | 1.108x | 1.184x |
| polyglot_key | 1.005x | 0.905x | 38.927x |
| polyglot_recompute | 1.009x | 0.994x | 0.955x |
| repetition_key | 1.452x | 1.205x | 20.200x |
| corpus_count | 1.033x | 1.009x | 1.228x |
| corpus_materialize | 1.009x | 1.000x | 1.158x |
| corpus_uci_sorted | 1.187x | 1.186x | 0.760x |
| replay_trusted_native | 1.204x | 1.138x | 0.603x |
| replay_checked_native | 1.253x | 1.143x | 0.969x |

Cold first legal move list: 127.477 ms → 0.010 ms (13201.82x).
