| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.03x | 2.76x | 1.55x | 1.35x | 1.83x | 1.30x | 29/30 |
| materialize | 1.04x | 2.20x | 1.74x | 1.70x | 2.66x | 1.89x | 30/30 |
| perft_bulk_d4 | 0.92x | 2.69x | 1.43x | 1.07x | 1.79x | 1.60x | 29/30 |
| perft_full_d3 | 0.96x | 1.48x | 1.71x | 1.54x | 2.11x | 1.75x | 30/30 |
| fen_parse | 1.04x | 1.74x | 1.51x | 1.20x | 1.56x | 1.28x | 30/30 |
| fen_write_native_ep | 0.99x | 3.05x | 9.77x | 1.91x | 27.27x | 2.29x | 30/30 |
| snapshot | 1.00x | 1.75x | 1.25x | 3.25x | 1.25x | 1.00x | 25/30 |
| san_parse_all | 0.91x | 1.74x | N/A | 4.05x | N/A | N/A | 12/12 |
| san_render_native | 0.98x | 9.01x | N/A | 2.10x | N/A | N/A | 12/12 |
| apply_children_full | 0.99x | 1.96x | 1.32x | 3.72x | 1.20x | 1.39x | 30/30 |
| make_unmake_full | 1.01x | N/A | N/A | 1.19x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.43x | 56.69x | 108.25x | 128.59x | 111.44x | 5/5 |
| chess960_materialize | 0.94x | 1.47x | 1.50x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.04x | 1.96x | 1.38x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.02x | 43.25x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.08x | 1.12x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.79x | 18.46x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.78x | 3.61x | 1.82x | 0.99x | 1.85x | 2.49x | 9/10 |
| corpus_materialize | 0.94x | 1.44x | 1.36x | 1.09x | 1.76x | 1.20x | 10/10 |
| corpus_uci_sorted | 1.00x | 1.96x | 2.92x | 2.24x | 2.88x | 1.55x | 10/10 |
| replay_trusted_native | 0.99x | 0.83x | 1.50x | 1.46x | 2.05x | 1.90x | 8/10 |
| replay_checked_native | 1.01x | 2.02x | 1.10x | 8.50x | 8.53x | 9.62x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
