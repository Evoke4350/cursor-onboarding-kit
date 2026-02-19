# syntax=docker/dockerfile:1.7
FROM node:22-alpine
WORKDIR /app
RUN --mount=type=cache,target=/root/.npm \
  npm install -g --no-audit --no-fund --omit=dev tsx@4
COPY src ./src
COPY ui ./ui
ENV NODE_ENV=production
ENV PORT=4200
ENV AGENT_DEV_KEY=lab-agent-key
# GITHUB_TOKEN intentionally absent.
# Without it, GitHub rate-limits to 60 req/hr.
# In a workshop with many students, this gets hit fast.
# Fix: docker run -e GITHUB_TOKEN=ghp_xxx ...
EXPOSE 4200
CMD ["tsx", "ui/server.ts"]
