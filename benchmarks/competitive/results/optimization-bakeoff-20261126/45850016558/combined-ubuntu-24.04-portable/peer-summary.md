| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 3.13x | 1.62x | 1.45x | 2.07x | 1.44x | 30/30 |
| materialize | 0.98x | 2.25x | 1.94x | 1.36x | 2.54x | 1.65x | 30/30 |
| perft_bulk_d4 | 0.97x | 2.99x | 1.65x | 1.29x | 2.05x | 1.85x | 30/30 |
| perft_full_d3 | 0.90x | 1.21x | 1.40x | 1.12x | 1.70x | 1.59x | 28/30 |
| fen_parse | 0.92x | 1.41x | 1.43x | 1.17x | 2.06x | 1.21x | 30/30 |
| fen_write_native_ep | 1.00x | 2.93x | 9.74x | 1.73x | 28.28x | 1.96x | 30/30 |
| snapshot | 1.00x | 1.69x | 1.20x | 2.90x | 1.02x | 0.99x | 23/30 |
| san_parse_all | 1.01x | 2.10x | N/A | 4.26x | N/A | N/A | 12/12 |
| san_render_native | 1.01x | 10.50x | N/A | 2.40x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 2.21x | 1.52x | 4.38x | 1.40x | 1.45x | 30/30 |
| make_unmake_full | 0.98x | N/A | N/A | 1.36x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.29x | 65.75x | 115.18x | 183.09x | 111.83x | 5/5 |
| chess960_materialize | 0.97x | 1.61x | 1.54x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.99x | 2.29x | 1.57x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 27.74x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.02x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.96x | 18.90x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.01x | 3.92x | 1.98x | 1.30x | 2.13x | 2.57x | 10/10 |
| corpus_materialize | 1.01x | 1.67x | 1.67x | 1.20x | 2.11x | 1.40x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.19x | 3.18x | 2.45x | 3.05x | 1.81x | 10/10 |
| replay_trusted_native | 1.00x | 0.85x | 1.63x | 2.00x | 2.56x | 2.41x | 9/10 |
| replay_checked_native | 1.01x | 2.13x | 1.06x | 7.56x | 7.47x | 8.55x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
