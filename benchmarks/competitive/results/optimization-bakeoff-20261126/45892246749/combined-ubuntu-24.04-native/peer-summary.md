| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.96x | 4.08x | 1.77x | 1.49x | 2.29x | 1.82x | 30/30 |
| materialize | 0.97x | 2.23x | 1.64x | 1.71x | 2.53x | 1.89x | 30/30 |
| perft_bulk_d4 | 0.85x | 3.16x | 1.30x | 0.99x | 1.95x | 1.57x | 27/30 |
| perft_full_d3 | 1.02x | 1.43x | 1.58x | 1.55x | 1.99x | 1.70x | 30/30 |
| fen_parse | 0.92x | 1.17x | 1.22x | 1.07x | 1.36x | 1.05x | 27/30 |
| fen_write_native_ep | 0.90x | 4.02x | 8.89x | 1.86x | 25.00x | 1.96x | 30/30 |
| snapshot | 0.86x | 1.58x | 1.00x | 3.75x | 1.00x | 1.16x | 24/30 |
| san_parse_all | 0.97x | 1.86x | N/A | 4.26x | N/A | N/A | 12/12 |
| san_render_native | 0.94x | 8.56x | N/A | 2.00x | N/A | N/A | 12/12 |
| apply_children_full | 0.88x | 2.00x | 1.29x | 3.94x | 1.29x | 1.56x | 30/30 |
| make_unmake_full | 0.78x | N/A | N/A | 0.93x | N/A | N/A | 1/6 |
| startpos | 0.91x | 1.02x | 82.35x | 152.10x | 181.22x | 148.30x | 5/5 |
| chess960_materialize | 0.92x | 1.71x | 1.62x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.86x | 2.14x | 1.30x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 39.98x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.97x | 0.97x | N/A | N/A | N/A | N/A | 0/2 |
| repetition_key | 1.10x | 21.96x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.96x | 5.06x | 1.90x | 1.05x | 2.31x | 2.12x | 9/10 |
| corpus_materialize | 1.02x | 1.40x | 1.31x | 1.07x | 1.72x | 1.18x | 10/10 |
| corpus_uci_sorted | 1.02x | 2.28x | 3.01x | 2.37x | 2.94x | 1.59x | 10/10 |
| replay_trusted_native | 0.99x | 0.83x | 1.55x | 1.56x | 2.18x | 2.54x | 9/10 |
| replay_checked_native | 0.92x | 2.12x | 1.09x | 8.34x | 7.99x | 9.49x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
