create table "questions" (
	"id" uuid not null,
	"title" varchar(255) not null,
	"description" text not null,
	"created_at" timestamp(0) without time zone not null default CURRENT_TIMESTAMP
);

alter table
	"questions"
add
	primary key ("id");
