| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.04x | 4.94x | 1.74x | 1.69x | 2.64x | 2.12x | 30/30 |
| materialize | 1.01x | 1.98x | 1.68x | 1.26x | 2.28x | 1.53x | 30/30 |
| perft_bulk_d4 | 1.04x | 4.15x | 1.66x | 1.51x | 2.49x | 2.07x | 30/30 |
| perft_full_d3 | 1.23x | 1.66x | 1.88x | 1.62x | 2.40x | 2.32x | 30/30 |
| fen_parse | 1.05x | 1.32x | 1.41x | 1.18x | 2.09x | 1.23x | 30/30 |
| fen_write_native_ep | 1.00x | 4.02x | 10.54x | 1.75x | 29.54x | 2.23x | 30/30 |
| snapshot | 0.72x | 1.43x | 0.90x | 2.63x | 0.90x | 0.90x | 12/30 |
| san_parse_all | 0.98x | 2.06x | N/A | 4.28x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 10.45x | N/A | 2.20x | N/A | N/A | 12/12 |
| apply_children_full | 1.01x | 2.33x | 1.33x | 4.64x | 1.48x | 1.92x | 30/30 |
| make_unmake_full | 1.01x | N/A | N/A | 1.28x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.25x | 77.91x | 144.14x | 229.96x | 149.41x | 5/5 |
| chess960_materialize | 1.00x | 1.60x | 1.46x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.99x | 2.83x | 1.52x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 26.07x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.98x | 1.02x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.98x | 19.39x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.07x | 5.85x | 1.93x | 1.48x | 2.84x | 2.59x | 10/10 |
| corpus_materialize | 1.01x | 1.59x | 1.56x | 1.15x | 2.01x | 1.37x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.38x | 3.23x | 2.41x | 3.16x | 1.70x | 10/10 |
| replay_trusted_native | 0.96x | 0.86x | 1.63x | 1.99x | 2.52x | 2.86x | 9/10 |
| replay_checked_native | 0.98x | 2.43x | 1.10x | 8.07x | 8.06x | 9.63x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
