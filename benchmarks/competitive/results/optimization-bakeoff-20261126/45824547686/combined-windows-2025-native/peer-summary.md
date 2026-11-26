| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.96x | 3.84x | 1.58x | 1.48x | 1.96x | 1.77x | 30/30 |
| materialize | 0.98x | 2.04x | 1.22x | 1.35x | 2.29x | 1.50x | 29/30 |
| perft_bulk_d4 | 0.98x | 3.41x | 1.36x | 1.11x | 1.76x | 1.62x | 29/30 |
| perft_full_d3 | 1.00x | 2.16x | 1.56x | 1.45x | 2.05x | 1.88x | 30/30 |
| fen_parse | 1.01x | 1.28x | 1.67x | 1.62x | 2.15x | 1.57x | 30/30 |
| fen_write_native_ep | 0.96x | 4.37x | 7.69x | 1.62x | 32.36x | 2.10x | 30/30 |
| snapshot | 0.94x | 2.19x | 1.19x | 5.62x | 1.10x | 1.10x | 27/30 |
| san_parse_all | 1.01x | 1.89x | N/A | 5.95x | N/A | N/A | 12/12 |
| san_render_native | 0.99x | 11.45x | N/A | 4.29x | N/A | N/A | 12/12 |
| apply_children_full | 0.98x | 2.06x | 1.33x | 10.88x | 1.41x | 1.44x | 30/30 |
| make_unmake_full | 0.95x | N/A | N/A | 1.31x | N/A | N/A | 6/6 |
| startpos | 0.90x | 1.00x | 91.21x | 218.79x | 268.36x | 215.40x | 4/5 |
| chess960_materialize | 0.95x | 1.53x | 1.23x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.01x | 2.53x | 1.42x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.99x | 32.02x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.93x | 1.08x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.88x | 14.08x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 4.37x | 1.77x | 1.00x | 1.94x | 1.93x | 9/10 |
| corpus_materialize | 0.99x | 1.42x | 1.18x | 1.12x | 1.81x | 1.25x | 10/10 |
| corpus_uci_sorted | 0.98x | 1.74x | 2.16x | 1.87x | 2.10x | 1.43x | 10/10 |
| replay_trusted_native | 0.93x | 0.76x | 1.60x | 1.78x | 2.19x | 2.28x | 8/10 |
| replay_checked_native | 1.02x | 2.18x | 1.04x | 7.90x | 7.29x | 9.04x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
