| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.02x | 4.49x | 1.57x | 1.59x | 2.13x | 2.04x | 29/30 |
| materialize | 0.94x | 1.92x | 1.62x | 1.02x | 2.21x | 1.32x | 27/30 |
| perft_bulk_d4 | 0.92x | 3.42x | 1.44x | 1.11x | 1.74x | 1.63x | 29/30 |
| perft_full_d3 | 1.04x | 1.63x | 1.83x | 1.48x | 2.15x | 2.28x | 30/30 |
| fen_parse | 1.00x | 1.25x | 1.40x | 1.35x | 1.71x | 1.44x | 30/30 |
| fen_write_native_ep | 0.96x | 4.24x | 9.28x | 1.68x | 29.23x | 2.11x | 30/30 |
| snapshot | 0.99x | 1.59x | 1.07x | 2.60x | 1.14x | 1.03x | 25/30 |
| san_parse_all | 0.97x | 1.85x | N/A | 3.89x | N/A | N/A | 12/12 |
| san_render_native | 0.99x | 12.32x | N/A | 2.81x | N/A | N/A | 12/12 |
| apply_children_full | 0.96x | 1.15x | 1.25x | 6.17x | 1.22x | 1.33x | 26/30 |
| make_unmake_full | 0.93x | N/A | N/A | 1.31x | N/A | N/A | 6/6 |
| startpos | 0.48x | 0.58x | 48.78x | 105.63x | 103.19x | 78.72x | 4/5 |
| chess960_materialize | 0.98x | 1.65x | 1.27x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.28x | 2.66x | 1.24x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.12x | 38.59x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.99x | 1.02x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.90x | 18.37x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.89x | 5.37x | 1.94x | 1.21x | 2.00x | 2.31x | 10/10 |
| corpus_materialize | 0.89x | 1.31x | 1.26x | 1.05x | 1.68x | 1.19x | 9/10 |
| corpus_uci_sorted | 1.04x | 2.31x | 2.96x | 2.00x | 2.53x | 1.46x | 10/10 |
| replay_trusted_native | 0.99x | 0.67x | 1.79x | 2.07x | 2.12x | 1.97x | 8/10 |
| replay_checked_native | 0.83x | 2.41x | 1.13x | 9.20x | 7.02x | 9.56x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
