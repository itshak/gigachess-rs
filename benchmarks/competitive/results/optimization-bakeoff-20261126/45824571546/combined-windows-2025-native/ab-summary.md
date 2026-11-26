| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.006x | 0.984x | 1.297x |
| materialize | 1.069x | 1.021x | 1.305x |
| perft_bulk_d4 | 1.023x | 0.979x | 0.987x |
| perft_full_d3 | 1.009x | 0.950x | 1.507x |
| fen_parse | 1.111x | 1.051x | 1.323x |
| fen_write_native_ep | 1.016x | 0.987x | 1.621x |
| snapshot | 1.263x | 1.244x | 1.262x |
| san_parse_all | 0.984x | 0.868x | 1.637x |
| san_render_native | 1.037x | 1.011x | 4.218x |
| apply_children_full | 1.011x | 0.975x | 1.235x |
| make_unmake_full | 1.070x | 1.023x | 1.221x |
| startpos | 0.964x | 0.964x | 1.010x |
| chess960_materialize | 1.062x | 0.989x | 1.283x |
| chess960_perft_d3 | 1.177x | 1.156x | 1.377x |
| polyglot_key | 1.016x | 0.979x | 43.326x |
| polyglot_recompute | 1.082x | 1.050x | 1.059x |
| repetition_key | 0.932x | 0.798x | 24.563x |
| corpus_count | 0.974x | 0.947x | 0.825x |
| corpus_materialize | 1.044x | 1.013x | 1.033x |
| corpus_uci_sorted | 0.971x | 0.932x | 1.495x |
| replay_trusted_native | 1.023x | 0.941x | 0.813x |
| replay_checked_native | 0.969x | 0.959x | 1.071x |

Cold first legal move list: 0.018 ms -> 0.019 ms (0.96x).
