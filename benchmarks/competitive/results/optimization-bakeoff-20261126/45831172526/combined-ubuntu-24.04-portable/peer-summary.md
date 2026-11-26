| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 2.89x | 1.48x | 1.35x | 1.90x | 1.33x | 30/30 |
| materialize | 1.03x | 1.99x | 1.78x | 1.27x | 2.37x | 1.55x | 30/30 |
| perft_bulk_d4 | 0.99x | 2.75x | 1.47x | 1.16x | 1.88x | 1.62x | 30/30 |
| perft_full_d3 | 1.01x | 1.42x | 1.64x | 1.34x | 1.99x | 1.81x | 30/30 |
| fen_parse | 1.03x | 1.38x | 1.38x | 1.05x | 2.02x | 1.17x | 28/30 |
| fen_write_native_ep | 1.00x | 3.14x | 9.50x | 1.72x | 28.22x | 1.98x | 30/30 |
| snapshot | 1.00x | 1.69x | 1.22x | 2.91x | 1.03x | 0.99x | 26/30 |
| san_parse_all | 1.00x | 2.01x | N/A | 4.33x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 10.40x | N/A | 2.26x | N/A | N/A | 12/12 |
| apply_children_full | 1.01x | 2.42x | 1.53x | 4.37x | 1.43x | 1.45x | 30/30 |
| make_unmake_full | 0.99x | N/A | N/A | 1.36x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.35x | 64.69x | 109.89x | 187.24x | 114.00x | 5/5 |
| chess960_materialize | 1.07x | 1.53x | 1.46x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.01x | 2.15x | 1.43x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.01x | 23.63x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.02x | 1.03x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.02x | 18.41x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 3.55x | 1.76x | 1.18x | 1.96x | 2.36x | 10/10 |
| corpus_materialize | 1.00x | 1.50x | 1.50x | 1.10x | 1.97x | 1.30x | 10/10 |
| corpus_uci_sorted | 0.99x | 2.18x | 3.17x | 2.41x | 3.07x | 1.70x | 10/10 |
| replay_trusted_native | 0.99x | 0.86x | 1.63x | 1.91x | 2.56x | 2.39x | 9/10 |
| replay_checked_native | 1.00x | 2.12x | 1.03x | 7.21x | 7.48x | 8.44x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
