| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.010x | 0.989x | 1.407x |
| materialize | 1.018x | 1.011x | 1.525x |
| perft_bulk_d4 | 0.862x | 0.825x | 1.031x |
| perft_full_d3 | 0.654x | 0.581x | 0.990x |
| fen_parse | 0.997x | 0.946x | 1.086x |
| fen_write_native_ep | 1.058x | 1.034x | 1.915x |
| snapshot | 0.706x | 0.702x | 1.152x |
| san_parse_all | 1.017x | 0.993x | 2.003x |
| san_render_native | 0.826x | 0.686x | 1.728x |
| apply_children_full | 0.579x | 0.532x | 0.612x |
| make_unmake_full | 0.670x | 0.574x | 0.754x |
| startpos | 0.906x | 0.906x | 1.282x |
| chess960_materialize | 0.943x | 0.942x | 1.364x |
| chess960_perft_d3 | 0.878x | 0.799x | 1.200x |
| polyglot_key | 1.088x | 0.999x | 39.203x |
| polyglot_recompute | 1.011x | 1.008x | 1.025x |
| repetition_key | 0.933x | 0.756x | 19.057x |
| corpus_count | 1.014x | 1.005x | 1.336x |
| corpus_materialize | 0.995x | 0.991x | 1.101x |
| corpus_uci_sorted | 1.014x | 1.010x | 1.577x |
| replay_trusted_native | 0.986x | 0.842x | 0.695x |
| replay_checked_native | 0.874x | 0.804x | 0.975x |

Cold first legal move list: 0.009 ms -> 0.010 ms (0.87x).
