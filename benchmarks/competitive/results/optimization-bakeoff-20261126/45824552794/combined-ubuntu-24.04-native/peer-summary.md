| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.01x | 4.34x | 1.55x | 1.50x | 2.33x | 1.81x | 30/30 |
| materialize | 1.02x | 2.08x | 1.71x | 1.12x | 2.13x | 1.45x | 29/30 |
| perft_bulk_d4 | 1.02x | 3.88x | 1.50x | 1.23x | 2.37x | 2.04x | 30/30 |
| perft_full_d3 | 0.99x | 1.25x | 1.28x | 0.97x | 1.84x | 1.72x | 26/30 |
| fen_parse | 1.00x | 1.41x | 1.60x | 1.21x | 2.44x | 1.31x | 30/30 |
| fen_write_native_ep | 1.00x | 4.04x | 9.92x | 1.81x | 26.96x | 2.14x | 30/30 |
| snapshot | 1.00x | 1.89x | 1.25x | 3.73x | 1.25x | 1.25x | 30/30 |
| san_parse_all | 1.00x | 2.29x | N/A | 3.85x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 10.40x | N/A | 2.07x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 1.90x | 1.26x | 4.77x | 1.45x | 1.79x | 30/30 |
| make_unmake_full | 0.98x | N/A | N/A | 1.17x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.20x | 102.56x | 137.31x | 238.54x | 146.37x | 5/5 |
| chess960_materialize | 1.00x | 1.85x | 1.48x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.01x | 2.95x | 1.50x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.01x | 20.51x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 0.97x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.00x | 15.62x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 4.27x | 1.63x | 1.20x | 2.17x | 1.88x | 9/10 |
| corpus_materialize | 1.01x | 1.84x | 1.70x | 1.18x | 2.70x | 1.48x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.31x | 3.12x | 2.32x | 2.95x | 1.65x | 10/10 |
| replay_trusted_native | 1.00x | 1.13x | 2.05x | 2.22x | 3.99x | 4.59x | 10/10 |
| replay_checked_native | 1.00x | 2.81x | 1.07x | 6.10x | 6.64x | 7.93x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
