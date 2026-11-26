| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.01x | 4.35x | 1.57x | 1.51x | 2.37x | 1.90x | 30/30 |
| materialize | 1.00x | 1.96x | 1.30x | 1.10x | 2.18x | 1.41x | 29/30 |
| perft_bulk_d4 | 0.94x | 3.49x | 1.43x | 1.25x | 2.16x | 1.97x | 30/30 |
| perft_full_d3 | 0.97x | 1.21x | 1.26x | 1.00x | 1.68x | 1.49x | 26/30 |
| fen_parse | 1.04x | 1.24x | 1.42x | 1.32x | 2.09x | 1.40x | 30/30 |
| fen_write_native_ep | 1.01x | 3.95x | 7.90x | 1.60x | 31.09x | 2.04x | 30/30 |
| snapshot | 0.84x | 1.58x | 1.21x | 2.71x | 1.21x | 1.04x | 29/30 |
| san_parse_all | 0.95x | 2.06x | N/A | 4.44x | N/A | N/A | 12/12 |
| san_render_native | 0.93x | 9.11x | N/A | 3.13x | N/A | N/A | 12/12 |
| apply_children_full | 0.97x | 1.48x | 1.02x | 6.04x | 1.14x | 1.30x | 27/30 |
| make_unmake_full | 0.93x | N/A | N/A | 1.14x | N/A | N/A | 5/6 |
| startpos | 1.00x | 1.08x | 80.35x | 144.57x | 206.44x | 149.20x | 5/5 |
| chess960_materialize | 1.01x | 1.51x | 1.23x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.95x | 2.55x | 1.35x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.98x | 20.67x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.97x | 0.96x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.94x | 16.58x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 4.56x | 1.82x | 1.33x | 2.39x | 2.18x | 10/10 |
| corpus_materialize | 0.98x | 1.74x | 1.36x | 1.15x | 2.76x | 1.45x | 10/10 |
| corpus_uci_sorted | 1.03x | 1.89x | 2.39x | 1.98x | 2.28x | 1.51x | 10/10 |
| replay_trusted_native | 1.11x | 0.77x | 1.33x | 1.64x | 2.76x | 3.32x | 8/10 |
| replay_checked_native | 0.93x | 2.05x | 0.90x | 5.02x | 5.84x | 7.05x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
