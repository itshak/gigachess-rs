| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.05x | 4.45x | 1.92x | 1.63x | 2.56x | 1.99x | 30/30 |
| materialize | 1.00x | 2.23x | 1.70x | 1.75x | 2.65x | 1.99x | 30/30 |
| perft_bulk_d4 | 1.07x | 3.97x | 1.63x | 1.24x | 2.47x | 1.98x | 29/30 |
| perft_full_d3 | 1.22x | 1.64x | 1.90x | 1.84x | 2.48x | 2.05x | 30/30 |
| fen_parse | 0.94x | 1.33x | 1.49x | 1.17x | 1.37x | 1.07x | 30/30 |
| fen_write_native_ep | 0.99x | 4.63x | 10.01x | 2.10x | 28.00x | 2.48x | 30/30 |
| snapshot | 1.00x | 2.00x | 1.00x | 4.67x | 1.00x | 1.33x | 26/30 |
| san_parse_all | 1.01x | 1.93x | N/A | 4.49x | N/A | N/A | 12/12 |
| san_render_native | 0.98x | 8.87x | N/A | 2.08x | N/A | N/A | 12/12 |
| apply_children_full | 0.99x | 2.11x | 1.33x | 4.02x | 1.42x | 1.62x | 30/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.21x | N/A | N/A | 6/6 |
| startpos | 0.89x | 1.00x | 78.56x | 211.42x | 171.06x | 141.84x | 5/5 |
| chess960_materialize | 0.88x | 1.54x | 1.53x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.02x | 2.54x | 1.53x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.05x | 39.39x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.62x | 1.62x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.29x | 23.89x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.18x | 6.30x | 2.39x | 1.28x | 2.88x | 2.66x | 10/10 |
| corpus_materialize | 1.00x | 1.39x | 1.31x | 1.08x | 1.70x | 1.18x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.23x | 2.90x | 2.21x | 2.78x | 1.54x | 10/10 |
| replay_trusted_native | 1.10x | 0.97x | 1.77x | 1.75x | 2.40x | 2.79x | 9/10 |
| replay_checked_native | 0.97x | 2.12x | 1.18x | 9.02x | 8.63x | 10.84x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
