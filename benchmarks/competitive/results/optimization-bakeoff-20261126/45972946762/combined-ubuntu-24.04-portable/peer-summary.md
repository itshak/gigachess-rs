| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 2.99x | 1.65x | 1.44x | 1.92x | 1.40x | 30/30 |
| materialize | 0.91x | 2.17x | 1.82x | 1.80x | 2.65x | 1.92x | 30/30 |
| perft_bulk_d4 | 0.99x | 3.01x | 1.58x | 1.18x | 1.98x | 1.79x | 29/30 |
| perft_full_d3 | 1.00x | 1.37x | 1.64x | 1.41x | 1.90x | 1.62x | 30/30 |
| fen_parse | 1.03x | 1.62x | 1.43x | 1.28x | 1.52x | 1.22x | 30/30 |
| fen_write_native_ep | 1.02x | 3.08x | 10.23x | 1.81x | 28.37x | 2.58x | 30/30 |
| snapshot | 1.00x | 1.75x | 1.25x | 3.25x | 1.25x | 1.00x | 28/30 |
| san_parse_all | 1.00x | 1.94x | N/A | 4.26x | N/A | N/A | 12/12 |
| san_render_native | 0.98x | 9.33x | N/A | 2.19x | N/A | N/A | 12/12 |
| apply_children_full | 1.01x | 2.25x | 1.48x | 4.29x | 1.38x | 1.60x | 30/30 |
| make_unmake_full | 1.06x | N/A | N/A | 1.28x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.36x | 62.09x | 110.09x | 120.72x | 102.09x | 5/5 |
| chess960_materialize | 1.00x | 1.82x | 1.77x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.01x | 2.13x | 1.50x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.01x | 41.13x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.06x | 1.02x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.04x | 21.01x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.03x | 4.26x | 2.13x | 1.17x | 2.23x | 2.97x | 10/10 |
| corpus_materialize | 1.00x | 1.51x | 1.41x | 1.14x | 1.87x | 1.25x | 10/10 |
| corpus_uci_sorted | 1.01x | 1.97x | 3.04x | 2.25x | 2.94x | 1.57x | 10/10 |
| replay_trusted_native | 0.95x | 0.89x | 1.61x | 1.53x | 2.13x | 1.99x | 9/10 |
| replay_checked_native | 1.01x | 2.07x | 1.20x | 8.96x | 8.79x | 9.92x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
