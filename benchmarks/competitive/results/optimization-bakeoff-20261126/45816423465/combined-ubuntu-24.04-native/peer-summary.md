| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.97x | 3.90x | 1.67x | 1.41x | 2.13x | 1.71x | 30/30 |
| materialize | 0.94x | 1.96x | 1.49x | 1.56x | 2.31x | 1.72x | 30/30 |
| perft_bulk_d4 | 0.84x | 3.28x | 1.34x | 1.02x | 1.99x | 1.61x | 27/30 |
| perft_full_d3 | 1.02x | 1.41x | 1.70x | 1.58x | 2.03x | 1.80x | 30/30 |
| fen_parse | 0.99x | 1.33x | 1.57x | 1.05x | 1.47x | 1.14x | 30/30 |
| fen_write_native_ep | 1.00x | 4.47x | 10.16x | 2.11x | 28.31x | 2.64x | 30/30 |
| snapshot | 1.67x | 2.00x | 1.00x | 4.67x | 1.00x | 1.33x | 25/30 |
| san_parse_all | 0.97x | 2.10x | N/A | 4.40x | N/A | N/A | 12/12 |
| san_render_native | 1.04x | 9.36x | N/A | 2.23x | N/A | N/A | 12/12 |
| apply_children_full | 1.03x | 1.89x | 1.18x | 3.57x | 1.21x | 1.47x | 30/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.12x | N/A | N/A | 6/6 |
| startpos | 1.50x | 1.25x | 98.58x | 171.35x | 218.81x | 181.96x | 5/5 |
| chess960_materialize | 0.99x | 1.57x | 1.52x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.03x | 2.36x | 1.42x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.93x | 37.42x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.04x | 1.07x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.99x | 19.10x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.64x | 4.15x | 1.56x | 0.84x | 1.87x | 1.71x | 8/10 |
| corpus_materialize | 0.94x | 1.36x | 1.26x | 1.03x | 1.66x | 1.13x | 9/10 |
| corpus_uci_sorted | 1.01x | 2.27x | 2.96x | 2.22x | 2.86x | 1.58x | 10/10 |
| replay_trusted_native | 1.01x | 0.83x | 1.47x | 1.44x | 2.02x | 2.40x | 9/10 |
| replay_checked_native | 0.99x | 2.09x | 1.16x | 8.67x | 8.40x | 10.03x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
