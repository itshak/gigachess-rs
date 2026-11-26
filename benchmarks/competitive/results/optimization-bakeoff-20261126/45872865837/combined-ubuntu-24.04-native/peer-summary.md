| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.01x | 4.30x | 1.84x | 1.56x | 2.44x | 1.88x | 30/30 |
| materialize | 1.03x | 2.18x | 1.65x | 1.70x | 2.55x | 1.94x | 30/30 |
| perft_bulk_d4 | 1.01x | 3.79x | 1.55x | 1.17x | 2.34x | 1.85x | 29/30 |
| perft_full_d3 | 1.24x | 1.77x | 1.93x | 1.87x | 2.50x | 2.14x | 30/30 |
| fen_parse | 0.93x | 1.26x | 1.49x | 0.98x | 1.39x | 1.06x | 24/30 |
| fen_write_native_ep | 0.99x | 4.46x | 10.03x | 2.10x | 27.75x | 2.66x | 30/30 |
| snapshot | 1.00x | 2.01x | 1.00x | 4.66x | 1.00x | 1.33x | 23/30 |
| san_parse_all | 1.01x | 1.92x | N/A | 4.45x | N/A | N/A | 12/12 |
| san_render_native | 0.99x | 8.99x | N/A | 2.15x | N/A | N/A | 12/12 |
| apply_children_full | 0.97x | 2.08x | 1.31x | 4.27x | 1.40x | 1.60x | 30/30 |
| make_unmake_full | 0.94x | N/A | N/A | 1.12x | N/A | N/A | 6/6 |
| startpos | 0.89x | 1.01x | 78.67x | 132.54x | 173.16x | 140.69x | 5/5 |
| chess960_materialize | 0.99x | 1.77x | 1.71x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.99x | 2.52x | 1.50x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.94x | 38.34x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.97x | 0.99x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.13x | 22.50x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.02x | 5.41x | 2.02x | 1.10x | 2.58x | 2.21x | 9/10 |
| corpus_materialize | 1.01x | 1.41x | 1.30x | 1.07x | 1.75x | 1.19x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.28x | 2.96x | 2.19x | 2.87x | 1.56x | 10/10 |
| replay_trusted_native | 1.02x | 0.89x | 1.60x | 1.55x | 2.21x | 2.57x | 9/10 |
| replay_checked_native | 1.00x | 2.18x | 1.21x | 9.11x | 8.98x | 10.59x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
