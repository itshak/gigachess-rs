| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.92x | 4.13x | 1.73x | 1.49x | 2.33x | 1.77x | 30/30 |
| materialize | 1.04x | 2.16x | 1.60x | 1.67x | 2.56x | 1.86x | 30/30 |
| perft_bulk_d4 | 0.95x | 3.62x | 1.43x | 1.12x | 2.23x | 1.77x | 29/30 |
| perft_full_d3 | 0.90x | 1.30x | 1.42x | 1.37x | 1.74x | 1.44x | 30/30 |
| fen_parse | 1.07x | 1.44x | 1.42x | 1.15x | 1.57x | 1.33x | 30/30 |
| fen_write_native_ep | 0.91x | 4.20x | 9.14x | 1.91x | 27.44x | 2.42x | 30/30 |
| snapshot | 0.75x | 2.26x | 1.75x | 3.01x | 1.75x | 1.50x | 30/30 |
| san_parse_all | 0.92x | 1.72x | N/A | 3.93x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 9.10x | N/A | 2.13x | N/A | N/A | 12/12 |
| apply_children_full | 0.93x | 2.02x | 1.35x | 3.79x | 1.24x | 1.65x | 30/30 |
| make_unmake_full | 0.98x | N/A | N/A | 1.17x | N/A | N/A | 6/6 |
| startpos | 1.12x | 2.01x | 99.58x | 170.53x | 216.85x | 193.80x | 5/5 |
| chess960_materialize | 0.97x | 1.67x | 1.55x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.94x | 2.36x | 1.39x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.02x | 39.96x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.95x | 0.96x | N/A | N/A | N/A | N/A | 0/2 |
| repetition_key | 1.02x | 24.30x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.98x | 5.36x | 1.99x | 1.10x | 2.43x | 2.22x | 9/10 |
| corpus_materialize | 0.99x | 1.41x | 1.30x | 1.07x | 1.70x | 1.16x | 10/10 |
| corpus_uci_sorted | 1.03x | 2.27x | 3.04x | 2.26x | 2.93x | 1.60x | 10/10 |
| replay_trusted_native | 0.98x | 0.88x | 1.60x | 1.54x | 2.20x | 2.56x | 9/10 |
| replay_checked_native | 0.89x | 1.92x | 1.03x | 8.13x | 7.82x | 9.38x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
