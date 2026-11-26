| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.980x | 0.962x | 1.307x |
| materialize | 0.982x | 0.958x | 1.146x |
| perft_bulk_d4 | 1.001x | 0.972x | 1.230x |
| perft_full_d3 | 1.116x | 0.958x | 0.983x |
| fen_parse | 1.215x | 1.165x | 1.456x |
| fen_write_native_ep | 0.979x | 0.951x | 1.534x |
| snapshot | 1.081x | 0.878x | 1.234x |
| san_parse_all | 0.937x | 0.798x | 1.864x |
| san_render_native | 1.022x | 0.988x | 3.700x |
| apply_children_full | 1.336x | 1.275x | 1.062x |
| make_unmake_full | 0.983x | 0.966x | 1.257x |
| startpos | 1.011x | 1.011x | 1.262x |
| chess960_materialize | 1.008x | 1.006x | 1.226x |
| chess960_perft_d3 | 0.995x | 0.990x | 1.305x |
| polyglot_key | 0.965x | 0.945x | 36.561x |
| polyglot_recompute | 0.996x | 0.978x | 0.970x |
| repetition_key | 0.926x | 0.855x | 24.945x |
| corpus_count | 1.008x | 0.993x | 1.261x |
| corpus_materialize | 0.973x | 0.915x | 1.139x |
| corpus_uci_sorted | 0.985x | 0.983x | 1.382x |
| replay_trusted_native | 0.977x | 0.945x | 0.653x |
| replay_checked_native | 1.024x | 0.980x | 0.965x |

Cold first legal move list: 0.023 ms -> 0.024 ms (0.97x).
