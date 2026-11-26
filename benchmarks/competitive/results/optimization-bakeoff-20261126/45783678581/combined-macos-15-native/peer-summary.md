| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.05x | 4.01x | 1.41x | 1.40x | 1.82x | 1.60x | 28/30 |
| materialize | 1.12x | 1.96x | 1.55x | 1.00x | 2.27x | 1.24x | 27/30 |
| perft_bulk_d4 | 1.02x | 4.22x | 1.59x | 1.33x | 2.07x | 2.06x | 30/30 |
| perft_full_d3 | 1.22x | 1.60x | 1.83x | 1.46x | 2.12x | 2.13x | 30/30 |
| fen_parse | 1.01x | 1.38x | 1.38x | 1.32x | 1.64x | 1.52x | 30/30 |
| fen_write_native_ep | 1.11x | 3.16x | 6.63x | 1.27x | 20.30x | 1.67x | 30/30 |
| snapshot | 1.03x | 1.60x | 1.18x | 2.34x | 1.13x | 0.99x | 25/30 |
| san_parse_all | 1.03x | 1.82x | N/A | 4.17x | N/A | N/A | 12/12 |
| san_render_native | 1.10x | 11.30x | N/A | 2.62x | N/A | N/A | 12/12 |
| apply_children_full | 1.38x | 1.26x | 1.38x | 6.28x | 1.33x | 1.47x | 30/30 |
| make_unmake_full | 1.18x | N/A | N/A | 1.30x | N/A | N/A | 6/6 |
| startpos | 1.03x | 1.21x | 70.30x | 156.55x | 175.48x | 144.46x | 5/5 |
| chess960_materialize | 1.08x | 1.67x | 1.45x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.01x | 2.91x | 1.37x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.88x | 39.05x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.93x | 1.05x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.00x | 19.30x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.98x | 6.25x | 1.96x | 1.36x | 2.36x | 3.07x | 9/10 |
| corpus_materialize | 0.90x | 1.44x | 1.30x | 1.24x | 1.86x | 1.15x | 10/10 |
| corpus_uci_sorted | 1.05x | 2.01x | 2.57x | 1.91x | 2.40x | 1.39x | 10/10 |
| replay_trusted_native | 1.21x | 0.67x | 1.69x | 1.93x | 2.23x | 2.01x | 8/10 |
| replay_checked_native | 1.13x | 2.20x | 1.21x | 11.23x | 9.74x | 10.64x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
