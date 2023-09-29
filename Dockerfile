FROM jitesoft/lighttpd:1.4.71

WORKDIR ./

COPY docker.conf /etc/lighttpd/lighttpd.conf

COPY --chown=www-data:www-data user/target/release/user /var/www/html/u
COPY server/document-root /var/www/html/
