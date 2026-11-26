| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 2.93x | 1.64x | 1.43x | 1.92x | 1.37x | 30/30 |
| materialize | 1.01x | 2.43x | 1.89x | 1.86x | 2.84x | 1.99x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.01x | 1.60x | 1.19x | 2.00x | 1.78x | 29/30 |
| perft_full_d3 | 1.00x | 1.29x | 1.52x | 1.36x | 1.81x | 1.52x | 30/30 |
| fen_parse | 0.89x | 1.34x | 1.28x | 1.08x | 1.34x | 1.06x | 30/30 |
| fen_write_native_ep | 1.01x | 3.18x | 10.14x | 1.85x | 28.71x | 2.54x | 30/30 |
| snapshot | 1.00x | 1.75x | 1.25x | 3.26x | 1.25x | 1.01x | 29/30 |
| san_parse_all | 1.09x | 1.96x | N/A | 4.52x | N/A | N/A | 12/12 |
| san_render_native | 0.97x | 8.93x | N/A | 2.07x | N/A | N/A | 12/12 |
| apply_children_full | 1.01x | 2.10x | 1.42x | 4.06x | 1.33x | 1.51x | 30/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.23x | N/A | N/A | 6/6 |
| startpos | 0.99x | 1.35x | 60.62x | 108.43x | 119.86x | 96.65x | 5/5 |
| chess960_materialize | 1.01x | 1.83x | 1.81x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.01x | 2.13x | 1.52x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.05x | 41.00x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.95x | 0.97x | N/A | N/A | N/A | N/A | 0/2 |
| repetition_key | 1.01x | 20.30x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 4.13x | 2.08x | 1.13x | 2.10x | 2.83x | 10/10 |
| corpus_materialize | 1.01x | 1.52x | 1.42x | 1.15x | 1.85x | 1.27x | 10/10 |
| corpus_uci_sorted | 0.99x | 1.95x | 2.94x | 2.21x | 2.83x | 1.53x | 10/10 |
| replay_trusted_native | 1.00x | 0.87x | 1.67x | 1.62x | 2.20x | 2.03x | 9/10 |
| replay_checked_native | 1.00x | 1.95x | 1.12x | 8.37x | 8.11x | 9.43x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
