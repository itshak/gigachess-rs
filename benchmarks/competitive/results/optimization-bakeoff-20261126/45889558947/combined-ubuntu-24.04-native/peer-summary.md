| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.73x | 1.73x | 1.68x | 2.58x | 2.00x | 30/30 |
| materialize | 1.05x | 2.15x | 1.75x | 1.19x | 2.20x | 1.50x | 30/30 |
| perft_bulk_d4 | 1.01x | 4.18x | 1.64x | 1.37x | 2.60x | 2.24x | 30/30 |
| perft_full_d3 | 0.99x | 1.51x | 1.55x | 1.22x | 2.24x | 1.88x | 30/30 |
| fen_parse | 1.00x | 1.37x | 1.53x | 1.19x | 2.15x | 1.24x | 30/30 |
| fen_write_native_ep | 0.99x | 4.06x | 10.26x | 1.85x | 28.58x | 2.18x | 30/30 |
| snapshot | 1.00x | 1.90x | 1.25x | 3.73x | 1.25x | 1.25x | 30/30 |
| san_parse_all | 0.99x | 2.35x | N/A | 3.85x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 10.22x | N/A | 2.09x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 2.01x | 1.30x | 4.83x | 1.51x | 1.75x | 30/30 |
| make_unmake_full | 0.97x | N/A | N/A | 1.19x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.20x | 104.07x | 138.24x | 233.83x | 147.57x | 5/5 |
| chess960_materialize | 1.00x | 1.91x | 1.55x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 3.13x | 1.61x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.99x | 20.51x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.99x | 1.01x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.00x | 19.10x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 4.36x | 1.77x | 1.30x | 2.37x | 2.05x | 9/10 |
| corpus_materialize | 1.00x | 1.88x | 1.77x | 1.23x | 2.86x | 1.55x | 10/10 |
| corpus_uci_sorted | 0.97x | 2.33x | 3.13x | 2.33x | 2.94x | 1.67x | 10/10 |
| replay_trusted_native | 1.08x | 1.19x | 2.09x | 2.32x | 4.02x | 4.82x | 10/10 |
| replay_checked_native | 1.01x | 2.88x | 1.13x | 6.28x | 7.10x | 8.44x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
