| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.14x | 4.95x | 1.74x | 1.71x | 2.51x | 2.20x | 30/30 |
| materialize | 1.00x | 1.98x | 1.35x | 1.24x | 2.31x | 1.55x | 30/30 |
| perft_bulk_d4 | 1.10x | 4.39x | 1.68x | 1.45x | 2.40x | 2.15x | 30/30 |
| perft_full_d3 | 1.03x | 1.91x | 1.50x | 1.34x | 1.96x | 1.89x | 30/30 |
| fen_parse | 1.05x | 1.34x | 1.45x | 1.35x | 2.27x | 1.47x | 30/30 |
| fen_write_native_ep | 1.02x | 4.22x | 8.31x | 1.56x | 32.05x | 1.99x | 30/30 |
| snapshot | 1.00x | 1.83x | 1.01x | 4.11x | 1.01x | 1.01x | 25/30 |
| san_parse_all | 1.03x | 2.07x | N/A | 5.74x | N/A | N/A | 12/12 |
| san_render_native | 1.02x | 11.28x | N/A | 3.87x | N/A | N/A | 12/12 |
| apply_children_full | 1.02x | 2.34x | 1.42x | 9.55x | 1.50x | 1.67x | 30/30 |
| make_unmake_full | 1.04x | N/A | N/A | 1.37x | N/A | N/A | 6/6 |
| startpos | 1.30x | 1.44x | 82.76x | 154.91x | 236.85x | 173.12x | 5/5 |
| chess960_materialize | 1.04x | 1.64x | 1.33x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.02x | 2.89x | 1.53x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.01x | 29.55x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.01x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.04x | 22.40x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.09x | 5.83x | 1.93x | 1.49x | 2.64x | 2.59x | 10/10 |
| corpus_materialize | 1.00x | 1.57x | 1.26x | 1.14x | 1.97x | 1.38x | 10/10 |
| corpus_uci_sorted | 1.01x | 1.78x | 2.23x | 1.83x | 2.21x | 1.44x | 10/10 |
| replay_trusted_native | 0.99x | 0.83x | 1.59x | 2.05x | 2.54x | 3.00x | 9/10 |
| replay_checked_native | 1.00x | 2.42x | 1.08x | 8.42x | 8.15x | 9.75x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
