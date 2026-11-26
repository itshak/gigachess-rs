| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.01x | 4.33x | 1.50x | 1.64x | 2.27x | 1.90x | 30/30 |
| materialize | 0.99x | 1.88x | 1.28x | 1.55x | 2.24x | 1.46x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.65x | 1.43x | 1.21x | 2.12x | 1.85x | 30/30 |
| perft_full_d3 | 1.05x | 1.38x | 1.45x | 1.29x | 1.89x | 1.83x | 30/30 |
| fen_parse | 1.00x | 1.31x | 1.43x | 1.37x | 2.29x | 1.46x | 30/30 |
| fen_write_native_ep | 1.02x | 4.36x | 8.97x | 1.67x | 33.39x | 2.04x | 30/30 |
| snapshot | 0.99x | 1.85x | 1.00x | 4.13x | 1.00x | 1.07x | 25/30 |
| san_parse_all | 0.96x | 2.04x | N/A | 5.43x | N/A | N/A | 12/12 |
| san_render_native | 0.97x | 10.67x | N/A | 3.87x | N/A | N/A | 12/12 |
| apply_children_full | 0.98x | 2.21x | 1.35x | 9.46x | 1.42x | 1.60x | 30/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.39x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.25x | 83.61x | 158.55x | 238.06x | 172.34x | 5/5 |
| chess960_materialize | 0.99x | 1.51x | 1.26x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.01x | 2.71x | 1.42x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 29.03x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.01x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.01x | 20.79x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.98x | 4.80x | 1.61x | 1.22x | 2.17x | 2.17x | 10/10 |
| corpus_materialize | 0.99x | 1.49x | 1.19x | 1.08x | 1.87x | 1.28x | 10/10 |
| corpus_uci_sorted | 1.01x | 1.82x | 2.26x | 1.85x | 2.21x | 1.44x | 10/10 |
| replay_trusted_native | 1.01x | 0.83x | 1.56x | 2.19x | 2.58x | 2.97x | 9/10 |
| replay_checked_native | 1.01x | 2.26x | 0.99x | 7.84x | 7.77x | 9.08x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
