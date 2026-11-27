| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.04x | 3.27x | 1.70x | 1.59x | 2.15x | 1.50x | 30/30 |
| materialize | 1.00x | 2.12x | 1.90x | 1.35x | 2.51x | 1.64x | 30/30 |
| perft_bulk_d4 | 1.06x | 3.26x | 1.77x | 1.37x | 2.21x | 1.92x | 30/30 |
| perft_full_d3 | 1.17x | 1.61x | 1.85x | 1.51x | 2.22x | 2.03x | 30/30 |
| fen_parse | 1.06x | 1.60x | 1.61x | 1.28x | 2.28x | 1.40x | 30/30 |
| fen_write_native_ep | 1.01x | 2.89x | 9.76x | 1.68x | 27.77x | 1.98x | 30/30 |
| snapshot | 1.00x | 1.70x | 1.23x | 3.05x | 1.01x | 1.07x | 30/30 |
| san_parse_all | 1.00x | 2.07x | N/A | 4.39x | N/A | N/A | 12/12 |
| san_render_native | 1.01x | 10.32x | N/A | 2.25x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 2.23x | 1.56x | 4.53x | 1.44x | 1.48x | 30/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.40x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.33x | 64.28x | 109.09x | 178.15x | 112.67x | 5/5 |
| chess960_materialize | 0.99x | 1.73x | 1.66x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.01x | 2.31x | 1.57x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.99x | 23.82x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.03x | 1.05x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.94x | 17.93x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.09x | 4.21x | 2.15x | 1.41x | 2.33x | 2.80x | 10/10 |
| corpus_materialize | 1.00x | 1.60x | 1.64x | 1.20x | 2.09x | 1.40x | 10/10 |
| corpus_uci_sorted | 1.06x | 2.25x | 3.31x | 2.51x | 3.21x | 1.85x | 10/10 |
| replay_trusted_native | 0.99x | 0.88x | 1.69x | 1.94x | 2.60x | 2.41x | 9/10 |
| replay_checked_native | 1.01x | 2.31x | 1.13x | 8.07x | 8.19x | 9.19x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
