| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 2.91x | 1.68x | 1.43x | 1.92x | 1.36x | 30/30 |
| materialize | 0.99x | 2.30x | 1.86x | 1.85x | 2.84x | 2.01x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.00x | 1.60x | 1.19x | 1.98x | 1.78x | 29/30 |
| perft_full_d3 | 1.00x | 1.36x | 1.59x | 1.45x | 1.90x | 1.61x | 30/30 |
| fen_parse | 0.93x | 1.48x | 1.37x | 1.04x | 1.54x | 1.10x | 30/30 |
| fen_write_native_ep | 1.01x | 2.99x | 9.82x | 1.82x | 27.30x | 2.22x | 30/30 |
| snapshot | 1.00x | 1.75x | 1.25x | 3.24x | 1.24x | 1.00x | 26/30 |
| san_parse_all | 1.00x | 1.92x | N/A | 4.34x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 9.59x | N/A | 2.18x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 2.21x | 1.46x | 4.21x | 1.36x | 1.58x | 30/30 |
| make_unmake_full | 1.02x | N/A | N/A | 1.23x | N/A | N/A | 6/6 |
| startpos | 0.99x | 1.35x | 56.98x | 101.05x | 129.07x | 98.31x | 5/5 |
| chess960_materialize | 0.94x | 1.76x | 1.66x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.15x | 1.53x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.02x | 42.43x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.04x | 1.05x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.95x | 21.38x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 4.14x | 2.10x | 1.13x | 2.10x | 2.83x | 10/10 |
| corpus_materialize | 1.00x | 1.52x | 1.41x | 1.15x | 1.86x | 1.25x | 10/10 |
| corpus_uci_sorted | 1.01x | 1.97x | 2.94x | 2.23x | 2.89x | 1.55x | 10/10 |
| replay_trusted_native | 0.99x | 0.85x | 1.62x | 1.60x | 2.23x | 2.02x | 9/10 |
| replay_checked_native | 1.01x | 2.07x | 1.14x | 9.00x | 8.64x | 9.98x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
