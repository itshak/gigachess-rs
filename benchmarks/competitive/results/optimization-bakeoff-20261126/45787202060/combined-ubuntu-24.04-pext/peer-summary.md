| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.99x | 4.24x | 1.77x | 1.53x | 2.40x | 1.81x | 30/30 |
| materialize | 1.04x | 2.16x | 1.61x | 1.68x | 2.59x | 1.88x | 30/30 |
| perft_bulk_d4 | 0.84x | 3.40x | 1.35x | 1.06x | 2.11x | 1.66x | 29/30 |
| perft_full_d3 | 0.68x | 1.03x | 1.22x | 1.18x | 1.53x | 1.25x | 26/30 |
| fen_parse | 0.93x | 1.12x | 1.18x | 1.05x | 1.31x | 1.01x | 23/30 |
| fen_write_native_ep | 1.00x | 4.66x | 10.06x | 1.90x | 29.49x | 2.16x | 30/30 |
| snapshot | 1.58x | 2.12x | 1.32x | 3.74x | 1.33x | 1.41x | 30/30 |
| san_parse_all | 1.15x | 1.95x | N/A | 4.38x | N/A | N/A | 12/12 |
| san_render_native | 0.91x | 8.25x | N/A | 1.93x | N/A | N/A | 12/12 |
| apply_children_full | 0.91x | 1.54x | 1.00x | 3.10x | 0.96x | 1.21x | 23/30 |
| make_unmake_full | 0.66x | N/A | N/A | 0.76x | N/A | N/A | 0/6 |
| startpos | 1.20x | 1.26x | 78.60x | 146.01x | 173.19x | 137.82x | 5/5 |
| chess960_materialize | 0.92x | 1.59x | 1.48x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.80x | 1.88x | 1.10x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.07x | 41.09x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.99x | 1.04x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.10x | 21.02x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.02x | 6.94x | 2.58x | 1.42x | 3.12x | 2.89x | 10/10 |
| corpus_materialize | 1.00x | 1.47x | 1.36x | 1.12x | 1.77x | 1.21x | 10/10 |
| corpus_uci_sorted | 0.99x | 2.23x | 2.99x | 2.21x | 2.92x | 1.55x | 10/10 |
| replay_trusted_native | 0.94x | 0.66x | 1.21x | 1.21x | 1.69x | 1.95x | 8/10 |
| replay_checked_native | 0.73x | 1.44x | 0.77x | 6.20x | 5.95x | 6.98x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
