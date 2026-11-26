| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.72x | 1.72x | 1.66x | 2.58x | 2.00x | 30/30 |
| materialize | 0.98x | 2.19x | 1.76x | 1.15x | 2.24x | 1.48x | 30/30 |
| perft_bulk_d4 | 1.00x | 4.32x | 1.70x | 1.32x | 2.54x | 2.18x | 30/30 |
| perft_full_d3 | 1.00x | 1.63x | 1.29x | 0.92x | 1.73x | 1.45x | 25/30 |
| fen_parse | 0.95x | 1.36x | 1.42x | 1.12x | 1.90x | 1.33x | 30/30 |
| fen_write_native_ep | 1.00x | 4.03x | 10.26x | 1.94x | 28.39x | 2.04x | 30/30 |
| snapshot | 1.00x | 1.90x | 1.25x | 3.70x | 1.25x | 1.25x | 30/30 |
| san_parse_all | 0.98x | 2.32x | N/A | 3.84x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 10.71x | N/A | 2.09x | N/A | N/A | 12/12 |
| apply_children_full | 1.03x | 2.02x | 1.31x | 4.84x | 1.53x | 1.75x | 30/30 |
| make_unmake_full | 0.98x | N/A | N/A | 1.22x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.20x | 101.55x | 143.53x | 218.20x | 164.86x | 5/5 |
| chess960_materialize | 1.00x | 1.91x | 1.55x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.99x | 3.10x | 1.60x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.01x | 20.58x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.09x | 1.06x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.01x | 18.91x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 4.38x | 1.77x | 1.29x | 2.36x | 2.13x | 9/10 |
| corpus_materialize | 0.99x | 1.89x | 1.79x | 1.22x | 2.86x | 1.60x | 10/10 |
| corpus_uci_sorted | 0.99x | 2.32x | 3.15x | 2.31x | 2.92x | 1.65x | 10/10 |
| replay_trusted_native | 0.83x | 0.96x | 1.66x | 1.88x | 3.23x | 3.99x | 9/10 |
| replay_checked_native | 1.04x | 2.74x | 1.09x | 6.03x | 7.14x | 8.45x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
