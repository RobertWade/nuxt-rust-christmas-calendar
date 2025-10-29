# Build
FROM node:20-alpine AS build
WORKDIR /app
COPY package.json package-lock.json* package-lock.yaml* yarn.lock* ./
RUN npm ci || yarn || npm i --frozen-lockfile=false
COPY . .
RUN npm run build || yarn build || npm build

# Runtime
FROM node:20-alpine
WORKDIR /app
ENV NODE_ENV=production
ENV PORT=3000
COPY --from=build /app/.output ./.output
EXPOSE 3000
CMD ["node", ".output/server/index.mjs"]
