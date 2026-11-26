| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 2.91x | 1.49x | 1.35x | 1.90x | 1.34x | 30/30 |
| materialize | 1.01x | 2.02x | 1.74x | 1.27x | 2.33x | 1.52x | 30/30 |
| perft_bulk_d4 | 1.02x | 2.79x | 1.50x | 1.16x | 1.89x | 1.64x | 30/30 |
| perft_full_d3 | 0.99x | 1.41x | 1.61x | 1.32x | 1.94x | 1.76x | 30/30 |
| fen_parse | 1.04x | 1.53x | 1.55x | 1.16x | 2.30x | 1.34x | 30/30 |
| fen_write_native_ep | 0.97x | 2.78x | 8.93x | 1.62x | 25.79x | 1.94x | 30/30 |
| snapshot | 1.00x | 1.70x | 1.23x | 3.06x | 1.01x | 1.07x | 30/30 |
| san_parse_all | 1.01x | 2.12x | N/A | 4.37x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 10.90x | N/A | 2.27x | N/A | N/A | 12/12 |
| apply_children_full | 1.01x | 2.21x | 1.53x | 4.47x | 1.42x | 1.46x | 30/30 |
| make_unmake_full | 0.99x | N/A | N/A | 1.35x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.32x | 64.39x | 105.14x | 181.27x | 115.83x | 5/5 |
| chess960_materialize | 0.99x | 1.49x | 1.42x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.01x | 2.18x | 1.46x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.02x | 27.62x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.03x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.03x | 19.60x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.98x | 3.58x | 1.76x | 1.18x | 1.93x | 2.35x | 10/10 |
| corpus_materialize | 0.98x | 1.49x | 1.49x | 1.11x | 1.93x | 1.28x | 10/10 |
| corpus_uci_sorted | 1.01x | 2.18x | 3.17x | 2.42x | 3.04x | 1.75x | 10/10 |
| replay_trusted_native | 1.00x | 0.86x | 1.65x | 1.87x | 2.54x | 2.36x | 9/10 |
| replay_checked_native | 1.01x | 2.12x | 1.03x | 7.40x | 7.38x | 8.27x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
