| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 3.88x | 1.68x | 1.42x | 2.19x | 1.73x | 30/30 |
| materialize | 0.99x | 2.09x | 1.59x | 1.65x | 2.50x | 1.80x | 30/30 |
| perft_bulk_d4 | 1.01x | 3.33x | 1.35x | 1.02x | 2.01x | 1.61x | 27/30 |
| perft_full_d3 | 0.98x | 1.43x | 1.68x | 1.59x | 2.09x | 1.76x | 30/30 |
| fen_parse | 1.04x | 1.25x | 1.54x | 1.13x | 1.44x | 1.22x | 30/30 |
| fen_write_native_ep | 1.01x | 4.70x | 10.23x | 1.92x | 28.39x | 2.25x | 30/30 |
| snapshot | 1.00x | 1.82x | 1.15x | 7.38x | 1.15x | 1.33x | 30/30 |
| san_parse_all | 1.00x | 1.94x | N/A | 4.34x | N/A | N/A | 12/12 |
| san_render_native | 1.02x | 9.05x | N/A | 2.13x | N/A | N/A | 12/12 |
| apply_children_full | 1.05x | 2.17x | 1.36x | 4.09x | 1.36x | 1.72x | 30/30 |
| make_unmake_full | 0.94x | N/A | N/A | 1.10x | N/A | N/A | 5/6 |
| startpos | 1.12x | 1.12x | 91.37x | 163.43x | 193.46x | 171.05x | 5/5 |
| chess960_materialize | 0.98x | 1.60x | 1.58x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.99x | 2.30x | 1.40x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 40.03x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.03x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.89x | 17.14x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 4.21x | 1.59x | 0.86x | 1.94x | 1.76x | 8/10 |
| corpus_materialize | 1.00x | 1.35x | 1.26x | 1.03x | 1.67x | 1.13x | 9/10 |
| corpus_uci_sorted | 1.00x | 2.22x | 2.93x | 2.24x | 2.79x | 1.55x | 10/10 |
| replay_trusted_native | 0.97x | 0.84x | 1.56x | 1.56x | 2.12x | 2.52x | 9/10 |
| replay_checked_native | 1.02x | 1.94x | 1.11x | 8.31x | 8.07x | 9.57x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
