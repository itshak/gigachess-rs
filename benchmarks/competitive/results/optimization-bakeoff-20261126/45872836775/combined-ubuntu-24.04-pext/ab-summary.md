| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.003x | 0.991x | 1.555x |
| materialize | 0.986x | 0.968x | 1.660x |
| perft_bulk_d4 | 1.016x | 0.999x | 1.205x |
| perft_full_d3 | 0.994x | 0.990x | 1.336x |
| fen_parse | 0.975x | 0.866x | 1.098x |
| fen_write_native_ep | 0.992x | 0.970x | 2.084x |
| snapshot | 1.000x | 0.996x | 0.999x |
| san_parse_all | 1.013x | 0.983x | 1.858x |
| san_render_native | 0.977x | 0.964x | 2.080x |
| apply_children_full | 1.000x | 0.994x | 1.353x |
| make_unmake_full | 1.000x | 0.979x | 1.188x |
| startpos | 0.893x | 0.893x | 0.999x |
| chess960_materialize | 0.944x | 0.929x | 1.407x |
| chess960_perft_d3 | 1.002x | 1.002x | 1.480x |
| polyglot_key | 1.005x | 1.000x | 39.446x |
| polyglot_recompute | 0.817x | 0.814x | 0.813x |
| repetition_key | 0.798x | 0.691x | 18.950x |
| corpus_count | 0.945x | 0.899x | 1.069x |
| corpus_materialize | 1.002x | 0.997x | 1.085x |
| corpus_uci_sorted | 0.992x | 0.988x | 1.541x |
| replay_trusted_native | 1.022x | 0.999x | 0.932x |
| replay_checked_native | 0.982x | 0.973x | 1.179x |

Cold first legal move list: 0.005 ms -> 0.006 ms (0.87x).
