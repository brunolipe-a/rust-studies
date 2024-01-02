create table "answers" (
	"id" uuid not null,
	"question_id" uuid not null,
	"content" text not null,
	"created_at" timestamp(0) without time zone not null default CURRENT_TIMESTAMP
);

alter table
	"answers"
add
	constraint "answers_question_id_foreign" foreign key ("question_id") references "questions" ("id");

alter table
	"answers"
add
	primary key ("id")
