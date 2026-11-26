| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.000x | 0.995x | 1.404x |
| materialize | 1.067x | 1.033x | 1.506x |
| perft_bulk_d4 | 0.911x | 0.860x | 1.101x |
| perft_full_d3 | 0.998x | 0.942x | 1.521x |
| fen_parse | 0.978x | 0.908x | 1.017x |
| fen_write_native_ep | 0.982x | 0.979x | 1.242x |
| snapshot | 0.948x | 0.943x | 0.864x |
| san_parse_all | 0.959x | 0.932x | 1.803x |
| san_render_native | 0.963x | 0.916x | 2.066x |
| apply_children_full | 0.686x | 0.660x | 0.784x |
| make_unmake_full | 0.836x | 0.784x | 0.974x |
| startpos | 1.091x | 1.091x | 1.322x |
| chess960_materialize | 0.995x | 0.993x | 1.430x |
| chess960_perft_d3 | 0.918x | 0.878x | 1.267x |
| polyglot_key | 1.018x | 0.997x | 39.446x |
| polyglot_recompute | 1.040x | 1.037x | 1.007x |
| repetition_key | 1.080x | 0.928x | 20.503x |
| corpus_count | 1.010x | 0.994x | 1.337x |
| corpus_materialize | 1.021x | 1.021x | 1.105x |
| corpus_uci_sorted | 1.002x | 0.999x | 1.559x |
| replay_trusted_native | 0.942x | 0.908x | 0.676x |
| replay_checked_native | 0.955x | 0.937x | 0.997x |

Cold first legal move list: 0.009 ms -> 0.009 ms (0.96x).
