| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.02x | 4.18x | 1.70x | 1.51x | 2.11x | 1.79x | 30/30 |
| materialize | 1.01x | 2.07x | 1.53x | 1.59x | 2.42x | 1.60x | 30/30 |
| perft_bulk_d4 | 0.99x | 4.01x | 1.59x | 1.23x | 2.28x | 2.04x | 30/30 |
| perft_full_d3 | 0.99x | 0.90x | 1.11x | 0.96x | 1.23x | 1.05x | 18/30 |
| fen_parse | 1.00x | 1.26x | 1.46x | 1.07x | 1.53x | 1.34x | 30/30 |
| fen_write_native_ep | 0.99x | 3.43x | 7.35x | 1.47x | 20.77x | 1.87x | 30/30 |
| snapshot | 0.70x | 1.50x | 1.16x | 1.99x | 1.15x | 0.99x | 25/30 |
| san_parse_all | 0.97x | 1.72x | N/A | 4.12x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 9.39x | N/A | 2.12x | N/A | N/A | 12/12 |
| apply_children_full | 1.02x | 1.53x | 0.99x | 3.01x | 0.92x | 1.24x | 21/30 |
| make_unmake_full | 1.00x | N/A | N/A | 0.94x | N/A | N/A | 0/6 |
| startpos | 1.20x | 1.99x | 103.22x | 164.84x | 223.24x | 195.50x | 5/5 |
| chess960_materialize | 1.01x | 1.56x | 1.36x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.48x | 1.32x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.01x | 40.40x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.02x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.03x | 25.86x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 6.81x | 2.65x | 1.35x | 3.04x | 2.89x | 10/10 |
| corpus_materialize | 1.02x | 1.48x | 1.36x | 1.13x | 1.77x | 1.22x | 10/10 |
| corpus_uci_sorted | 1.00x | 1.42x | 1.84x | 1.38x | 1.80x | 0.97x | 8/10 |
| replay_trusted_native | 0.99x | 0.60x | 1.10x | 1.06x | 1.51x | 1.75x | 7/10 |
| replay_checked_native | 0.88x | 1.70x | 0.86x | 6.81x | 6.54x | 7.61x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
