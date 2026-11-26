| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.02x | 2.79x | 1.56x | 1.35x | 1.80x | 1.29x | 29/30 |
| materialize | 1.00x | 2.19x | 1.73x | 1.73x | 2.62x | 1.88x | 30/30 |
| perft_bulk_d4 | 0.99x | 2.70x | 1.44x | 1.07x | 1.78x | 1.60x | 29/30 |
| perft_full_d3 | 1.01x | 1.49x | 1.73x | 1.55x | 2.07x | 1.76x | 30/30 |
| fen_parse | 0.98x | 1.56x | 1.47x | 1.19x | 1.62x | 1.18x | 30/30 |
| fen_write_native_ep | 0.98x | 3.03x | 9.89x | 2.03x | 27.30x | 2.19x | 30/30 |
| snapshot | 1.00x | 1.75x | 1.25x | 3.27x | 1.26x | 1.00x | 30/30 |
| san_parse_all | 1.07x | 1.89x | N/A | 4.13x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 9.30x | N/A | 2.16x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 1.98x | 1.30x | 3.76x | 1.21x | 1.41x | 30/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.16x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.36x | 56.99x | 110.04x | 125.56x | 96.40x | 5/5 |
| chess960_materialize | 0.96x | 1.53x | 1.54x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.99x | 1.98x | 1.39x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.95x | 42.08x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.98x | 1.04x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.96x | 17.36x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.96x | 3.63x | 1.83x | 1.01x | 1.84x | 2.49x | 9/10 |
| corpus_materialize | 1.00x | 1.44x | 1.35x | 1.08x | 1.76x | 1.19x | 10/10 |
| corpus_uci_sorted | 1.01x | 1.94x | 2.94x | 2.21x | 2.87x | 1.56x | 10/10 |
| replay_trusted_native | 1.02x | 0.83x | 1.58x | 1.56x | 2.15x | 1.95x | 9/10 |
| replay_checked_native | 0.98x | 1.96x | 1.09x | 8.40x | 8.37x | 9.43x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
