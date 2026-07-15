# Security Policy

The bootstrap is local and stdio-only. It provides no network listener, shell access, arbitrary execution, secret storage, or persistent state. Plugin data must live outside skill roots; skill roots are read-only inputs. Fail-closed filesystem path handling (allowed roots, traversal rejection, and symlink-escape prevention) is planned but not implemented by this bootstrap.

Report suspected vulnerabilities privately to the project maintainers with the affected version, reproduction steps, and impact.
