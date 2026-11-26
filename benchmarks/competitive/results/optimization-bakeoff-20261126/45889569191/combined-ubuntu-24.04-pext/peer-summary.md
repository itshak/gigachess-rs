| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.02x | 4.60x | 1.92x | 1.68x | 2.53x | 1.96x | 30/30 |
| materialize | 1.08x | 2.30x | 1.77x | 1.79x | 2.93x | 1.92x | 30/30 |
| perft_bulk_d4 | 1.02x | 3.90x | 1.56x | 1.22x | 2.37x | 1.89x | 29/30 |
| perft_full_d3 | 1.25x | 1.68x | 1.91x | 1.87x | 2.44x | 2.01x | 30/30 |
| fen_parse | 1.00x | 1.48x | 1.57x | 1.16x | 1.45x | 1.13x | 30/30 |
| fen_write_native_ep | 1.01x | 4.99x | 10.22x | 2.04x | 28.40x | 2.56x | 30/30 |
| snapshot | 1.00x | 1.67x | 1.33x | 4.00x | 1.33x | 1.33x | 30/30 |
| san_parse_all | 0.91x | 1.71x | N/A | 3.92x | N/A | N/A | 12/12 |
| san_render_native | 1.03x | 9.26x | N/A | 2.14x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 2.12x | 1.33x | 4.07x | 1.25x | 1.72x | 30/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.20x | N/A | N/A | 6/6 |
| startpos | 1.25x | 1.25x | 99.35x | 182.65x | 214.53x | 178.12x | 5/5 |
| chess960_materialize | 1.06x | 1.76x | 1.68x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.52x | 1.49x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 38.60x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.09x | 1.09x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.98x | 22.43x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.95x | 5.15x | 1.92x | 1.07x | 2.38x | 2.17x | 10/10 |
| corpus_materialize | 1.02x | 1.44x | 1.33x | 1.09x | 1.78x | 1.19x | 10/10 |
| corpus_uci_sorted | 1.03x | 2.32x | 3.00x | 2.24x | 2.97x | 1.58x | 10/10 |
| replay_trusted_native | 1.03x | 0.91x | 1.68x | 1.66x | 2.30x | 2.62x | 9/10 |
| replay_checked_native | 1.00x | 2.18x | 1.17x | 9.12x | 9.02x | 10.39x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
